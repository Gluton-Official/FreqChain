/// The rate of change for the value compared to the drag distance
const DRAG_SENSITIVITY: f32 = 0.005;

/// The multiplier applied to [DRAG_SENSITIVITY] when granular dragging
const GRANULAR_SENSITIVITY_MULTIPLIER: f32 = 0.1;

#[derive(Debug, Clone, Copy)]
pub struct DragState {
    initial_position: f32,
    initial_value: f32,

    granular_anchor: Option<f32>,

    // /// When dragging past the max or min value, requires dragging an equal amount
    // /// in the opposite direction, unless released, before affecting the value.
    // pub allow_over_drag: bool,
}

impl DragState {
    /// `initial_value` must be within 0.0 and 1.0
    pub fn new(initial_position: f32, initial_value: f32) -> Self {
        nih_debug_assert!(initial_value >= 0.0 && initial_value <= 1.0, "initial_value must be within 0.0 and 1.0: {}", initial_value);
        Self {
            initial_position,
            initial_value,

            granular_anchor: None,

            // allow_over_drag: false,
        }
    }

    /// `initial_value` must be within 0.0 and 1.0
    pub fn new_and_start_granular(initial_position: f32, initial_value: f32) -> Self {
        let mut this = Self::new(initial_position, initial_value);
        this.granular_anchor = Some(initial_position);
        this
    }

    pub fn start_granular(&mut self, granular_anchor: f32) {
        if self.granular_anchor.is_none() {
            self.granular_anchor = Some(granular_anchor);
        }
    }

    pub fn stop_granular(&mut self, current_position: f32) {
        if let Some(granular_anchor) = self.granular_anchor {
            let granular_drag_distance = current_position - granular_anchor;
            self.initial_position += granular_drag_distance - granular_drag_distance * GRANULAR_SENSITIVITY_MULTIPLIER;

            self.granular_anchor = None;
        }
    }

    pub fn value(&mut self, current_position: f32) -> f32 {
        let mut drag_distance = if let Some(granular_anchor) = self.granular_anchor {
            (granular_anchor - self.initial_position) + (current_position - granular_anchor) * GRANULAR_SENSITIVITY_MULTIPLIER
        } else {
            current_position - self.initial_position
        };
        // Negate to correct Y being upside down
        drag_distance = -drag_distance;

        let value_delta = drag_distance * DRAG_SENSITIVITY;
        self.initial_value + value_delta
    }
}