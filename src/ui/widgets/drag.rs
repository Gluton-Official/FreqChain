use std::ops::Range;
use nih_plug_iced::{Point, Rectangle, Vector};
use crate::util::remap::normalize_ranged;

/// The distance multiplier when granular dragging
const GRANULAR_SENSITIVITY_MULTIPLIER: f32 = 0.1;

pub trait DragTrait {
    type Position;
    type Value;
    type Range;

    fn new(initial_position: Self::Position, initial_value: Self::Value) -> Self;
    fn new_and_start_granular(initial_position: Self::Position, initial_value: Self::Value) -> Self;
    fn start_granular(&mut self, granular_anchor: Self::Position);
    fn stop_granular(&mut self, current_position: Self::Position);
    fn value(&mut self, bounds: Self::Range, current_position: Self::Position) -> Self::Value;
}

#[derive(Debug, Clone, Copy)]
pub struct DragState {
    initial_position: f32,
    initial_value: f32,

    granular_anchor: Option<f32>,

    // /// When dragging past the max or min value, requires dragging an equal amount
    // /// in the opposite direction, unless released, before affecting the value.
    // pub allow_over_drag: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct DragState2D {
    initial_position: Point,
    initial_value: Vector,

    granular_anchor: Option<Point>,

    // /// When dragging past the max or min value, requires dragging an equal amount
    // /// in the opposite direction, unless released, before affecting the value.
    // pub allow_over_drag: bool,
}

impl DragTrait for DragState {
    type Position = f32;
    type Value = f32;
    type Range = Range<f32>;

    /// `initial_value` must be within 0.0 and 1.0
    fn new(initial_position: f32, initial_value: f32) -> Self {
        nih_debug_assert!(initial_value >= 0.0 && initial_value <= 1.0, "initial_value must be within 0.0 and 1.0: {}", initial_value);
        Self {
            initial_position,
            initial_value,

            granular_anchor: None,

            // allow_over_drag: false,
        }
    }

    /// `initial_value` must be within 0.0 and 1.0
    fn new_and_start_granular(initial_position: f32, initial_value: f32) -> Self {
        let mut this = Self::new(initial_position, initial_value);
        this.granular_anchor = Some(initial_position);
        this
    }

    fn start_granular(&mut self, granular_anchor: f32) {
        if self.granular_anchor.is_none() {
            self.granular_anchor = Some(granular_anchor);
        }
    }

    fn stop_granular(&mut self, current_position: f32) {
        if let Some(granular_anchor) = self.granular_anchor {
            let granular_drag_distance = current_position - granular_anchor;
            self.initial_position += granular_drag_distance - granular_drag_distance * GRANULAR_SENSITIVITY_MULTIPLIER;

            self.granular_anchor = None;
        }
    }

    fn value(&mut self, bounds: Range<f32>, current_position: f32) -> f32 {
        let mut drag_distance = if let Some(granular_anchor) = self.granular_anchor {
            (granular_anchor - self.initial_position) + (current_position - granular_anchor) * GRANULAR_SENSITIVITY_MULTIPLIER
        } else {
            current_position - self.initial_position
        };
        // Negate to correct Y being upside down
        drag_distance = -drag_distance;

        // let value_delta = drag_distance * DRAG_SENSITIVITY;
        let value_delta = normalize_ranged(drag_distance, &(0f32..(bounds.end - bounds.start)));
        self.initial_value + value_delta
    }
}

impl DragTrait for DragState2D {
    type Position = Point;
    type Value = Vector;
    type Range = Rectangle;

    /// `initial_value` must be within 0.0 and 1.0
    fn new(initial_position: Point, initial_value: Vector) -> Self {
        nih_debug_assert!(
            (0_f32..=1_f32).contains(&initial_value.x) && (0_f32..=1_f32).contains(&initial_value.y),
            "Initial values must be within 0.0 and 1.0: {:?}", initial_value
        );
        Self {
            initial_position,
            initial_value,

            granular_anchor: None,

            // allow_over_drag: false,
        }
    }

    /// `initial_value` must be within 0.0 and 1.0
    fn new_and_start_granular(initial_position: Point, initial_value: Vector) -> Self {
        let mut this = Self::new(initial_position, initial_value);
        this.granular_anchor = Some(initial_position);
        this
    }

    fn start_granular(&mut self, granular_anchor: Point) {
        if self.granular_anchor.is_none() {
            self.granular_anchor = Some(granular_anchor);
        }
    }

    fn stop_granular(&mut self, current_position: Point) {
        if let Some(granular_anchor) = self.granular_anchor {
            let granular_drag_distance = current_position - granular_anchor;
            self.initial_position = self.initial_position + (granular_drag_distance - granular_drag_distance * GRANULAR_SENSITIVITY_MULTIPLIER);

            self.granular_anchor = None;
        }
    }

    fn value(&mut self, bounds: Rectangle, current_position: Point) -> Vector {
        let mut drag_distance = if let Some(granular_anchor) = self.granular_anchor {
            (granular_anchor - self.initial_position) + (current_position - granular_anchor) * GRANULAR_SENSITIVITY_MULTIPLIER
        } else {
            current_position - self.initial_position
        };
        // Negate to correct Y being upside down
        drag_distance = Vector::new(drag_distance.x, -drag_distance.y);

        let value_delta = Vector {
            x: drag_distance.x / bounds.width,
            y: drag_distance.y / bounds.height,
        };
        self.initial_value + value_delta
    }
}
