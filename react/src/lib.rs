use std::collections::{BTreeSet, HashMap};

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputCellId(usize);
/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ComputeCellId(usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CallbackId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

impl From<CellId> for usize {
    fn from(value: CellId) -> Self {
        match value {
            CellId::Input(InputCellId(id)) => id,
            CellId::Compute(ComputeCellId(id)) => id
        }
    }
}

impl From<ComputeCellId> for usize {
    fn from(value: ComputeCellId) -> Self {
        value.0
    }
}

impl From<InputCellId> for usize {
    fn from(value: InputCellId) -> Self {
        value.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

type BoxedCallback<'a ,T> = Box<dyn FnMut(T) + 'a>;
type BoxedComputeFn<'a, T> = Box<dyn Fn(&[T]) -> T + 'a>;

#[derive(Default)]
pub struct Reactor<'a, T> {
    id_counter: usize,
    // All cell values as a flat map
    values: HashMap<usize, T>,
    // Compute cells metadata storage
    compute_cells: HashMap<ComputeCellId, ComputeCell<'a,T>>,
    // List of downstream dependencies of each cell
    downstream: HashMap<usize, Vec<ComputeCellId>>,
    // Callbacks storage
    callbacks: HashMap<ComputeCellId, Vec<(CallbackId, BoxedCallback<'a, T>)>>,
}
// Compute cell metadata
struct ComputeCell<'a, T> {
    dependencies: Vec<CellId>,
    compute_func: BoxedComputeFn<'a, T>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq + Default> Reactor<'a, T> {
    pub fn new() -> Self {
        Self::default()
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        let id = self.next_id();
        self.values.insert(id, initial);
        InputCellId(id)
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'a>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        let id = self.next_id();
        let cell_id = ComputeCellId(id);
        let metadata = ComputeCell {
            dependencies: dependencies.to_vec(),
            compute_func: Box::new(compute_func),
        };

        let value = (metadata.compute_func)(self.cell_values(dependencies)?.as_slice());
        self.values.insert(id, value);
        self.compute_cells.insert(cell_id, metadata);

        // register downstream dependencies
        // at this point we already ensured that all dependency cells exist
        for &d in dependencies {
            self.downstream.entry(d.into()).or_default().push(cell_id);
        }

        Ok(cell_id)
    }

    fn cell_values(&self, idx: &[CellId]) -> Result<Vec<T>, CellId> {
        idx.iter()
            .map(|&id| self.values.get(&id.into()).copied().ok_or(id))
            .collect()
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        self.values.get(&id.into()).copied()
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        if let Some(value) = self.values.get_mut(&id.into()) {
            if *value == new_value {
                return true; // No change
            }
            *value = new_value;
        } else { return false; }

        // Dependencies evaluation priority queue
        let mut evaluation_queue = BTreeSet::new();

        // Add direct dependencies of the current cell
        if let Some(deps) = self.downstream.get(&id.into()) {
            for &dep_id in deps {
                evaluation_queue.insert(dep_id);
            }
        }

        // Dequeue dependencies and process them in ascending order
        // (ids are sequential, so actually in creation order )
        while let Some(current_id) = evaluation_queue.pop_first() {

            let compute_cell_metadata = &self.compute_cells[&current_id];
            let dep_values: Vec<T> = compute_cell_metadata.dependencies.iter()
                .map(|&dep| self.values[&dep.into()])
                .collect();
            let old_value = self.values[&current_id.into()];
            let new_calculated_value = (compute_cell_metadata.compute_func)(&dep_values);

            // If the value has actually changed then add all downstream dependencies into the queue
            if old_value != new_calculated_value {
                self.values.insert(current_id.into(), new_calculated_value);
                self.execute_callbacks(current_id, new_calculated_value);

                if let Some(next_deps) = self.downstream.get(&current_id.into()) {
                    for &next_id in next_deps {
                        evaluation_queue.insert(next_id);
                    }
                }
            }
        }
        true
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        let callback_id = CallbackId(self.next_id());
        if self.compute_cells.contains_key(&id) {
            self.callbacks
                .entry(id)
                .or_default()
                .push((callback_id, Box::new(callback)));
            Some(callback_id)
        } else { None }
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        if !self.compute_cells.contains_key(&cell) {
            return Err(RemoveCallbackError::NonexistentCell);
        }

        if let Some(callbacks) = self.callbacks.get_mut(&cell) && let Some(index) = callbacks.iter().position(|(c, _)| *c == callback) {
            // We don't guarantee callbacks order
            let _ = callbacks.swap_remove(index);
            return Ok(());
        }
        Err(RemoveCallbackError::NonexistentCallback)
    }

    // Produces next sequential id
    fn next_id(&mut self) -> usize {
        self.id_counter += 1;
        self.id_counter
    }

    /// Executes callbacks for a given `ComputeCellId` if it exists
    fn execute_callbacks(&mut self, cell_id: ComputeCellId, new_value: T) {
        if let Some(callbacks) = self.callbacks.get_mut(&cell_id) {
            for (_id, callback) in callbacks.iter_mut() {
                callback(new_value);
            }
        }
    }
}
