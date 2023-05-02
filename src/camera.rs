use crate::{
    rtweekend::{Degrees, Radians},
    vec3,
    vector3::Vec3,
    Ray,
};

/// A builder to create a Camera
pub struct CameraBuilder {
    look_from: Vec3,
    look_at: Vec3,
    up_vector: Vec3,
    vertical_fov: Degrees,
    aspect_ratio: f64,
    aperture: f64,
    focus_distance: f64,
}

impl CameraBuilder {
    /// Point from which the camera will look from
    pub fn look_from(mut self, point: Vec3) -> CameraBuilder {
        self.look_from = point;
        self
    }
    /// Point at which the camera will look at
    pub fn look_at(mut self, point: Vec3) -> CameraBuilder {
        self.look_at = point;
        self
    }
    /// Vector which specifies where it's up for the camera
    pub fn up_vector(mut self, up_vector: Vec3) -> CameraBuilder {
        self.up_vector = up_vector;
        self
    }
    /// Vertical field of view for the camera in degrees
    pub fn vertical_fov(mut self, field_of_view: Degrees) -> CameraBuilder {
        self.vertical_fov = field_of_view;
        self
    }
    /// Aspect ratio of the camera
    pub fn aspect_ratio(mut self, aspect_ratio: f64) -> CameraBuilder {
        self.aspect_ratio = aspect_ratio;
        self
    }
    /// Aperture of the camera
    pub fn aperture(mut self, aperture: f64) -> CameraBuilder {
        self.aperture = aperture;
        self
    }
    /// Focus distance of the camera
    pub fn focus_distance(mut self, focus_distance: f64) -> CameraBuilder {
        self.focus_distance = focus_distance;
        self
    }
    /// Create a Camera with the supplied values. If some values were not
    /// supplied by the user, the default ones will be used instead. Calling
    /// this function again will create the same Camera.
    pub fn build(&self) -> Camera {
        let theta: Radians = self.vertical_fov.into();
        let h = (theta.as_f64() / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = self.aspect_ratio * viewport_height;

        // `w` is the opposite direction of where the camera looks at
        let w = (self.look_from - self.look_at).unit_vec();
        let u = self.up_vector.cross(w).unit_vec();
        let v = w.cross(u);

        let origin = self.look_from;
        let horizontal = self.focus_distance * viewport_width * u;
        let vertical = self.focus_distance * viewport_height * v;
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - self.focus_distance * w;

        let lens_radius = self.aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            _w: w,
            lens_radius,
            aspect_ratio: self.aspect_ratio,
        }
    }
}

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    _w: Vec3,
    lens_radius: f64,
    aspect_ratio: f64,
}

impl Camera {
    /// Return a Camera builder
    pub fn builder() -> CameraBuilder {
        // fill with default values
        CameraBuilder {
            look_from: Vec3::zero(),
            look_at: vec3!(0.0, -1.0, 0.0),
            up_vector: vec3!(0.0, 1.0, 0.0),
            vertical_fov: 90.0.into(),
            aspect_ratio: 16.0 / 9.0,
            aperture: 0.0,
            focus_distance: 1.0,
        }
    }
    /// Create a Camera with default values
    pub fn new() -> Camera {
        Camera::builder().build()
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        if self.lens_radius == 0.0 {
            return Ray::new(
                self.origin,
                self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
            );
        }

        let rd = self.lens_radius * Vec3::random_in_unit_disc();
        let offset = self.u * rd.x + self.v * rd.y;
        let origin_with_offset = self.origin + offset;

        Ray::new(
            origin_with_offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - origin_with_offset,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
