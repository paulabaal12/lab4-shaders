use nalgebra_glm::{Vec3, Vec4, Mat3, mat4_to_mat3};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;
use crate::CelestialBody;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = Vec4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );

    let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

    let w = transformed.w;
    let transformed_position = Vec4::new(
        transformed.x / w,
        transformed.y / w,
        transformed.z / w,
        1.0
    );

    let screen_position = uniforms.viewport_matrix * transformed_position;

    let model_mat3 = mat4_to_mat3(&uniforms.model_matrix);
    let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

    let transformed_normal = normal_matrix * vertex.normal;

    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
        transformed_normal: transformed_normal
    }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  match uniforms.current_body {
      CelestialBody::Sun => sun_shader(fragment, uniforms),
      CelestialBody::RockyPlanet => rocky_planet_shader(fragment, uniforms),
      CelestialBody::GasGiant => gas_giant_shader(fragment, uniforms),
      CelestialBody::CloudyPlanet => cloudy_planet_shader(fragment, uniforms),
      CelestialBody::RingedPlanet => ring_shader(fragment, uniforms),
      CelestialBody::IcePlanet => ice_planet_shader(fragment, uniforms),
      CelestialBody::ColorPlanet => colorful_planet_shader(fragment, uniforms),
      CelestialBody::Moon => moon_shader(fragment, uniforms),
  }
}

fn colorful_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let position = fragment.vertex_position;
    let time = uniforms.time as f32 * 0.01;


    let color1 = Color::new(245, 56, 121);   
    let color2 = Color::new(245, 140, 105); 
    let color3 = Color::new(245, 115, 105); 
    let color4 = Color::new(245, 105, 238); 
    let color5 = Color::new(245, 159, 95);  
    let color6 = Color::new(245, 168, 162); 

    //Anillos
    let ring1_color = Color::new(245, 7, 123); 
    let ring2_color = Color::new(245, 166, 195);  

    // Patrón
    let curve_pattern = uniforms.noise.get_noise_3d(
        position.x * 5.0 + time * 1.5,
        position.y * 5.0,
        position.z * 5.0
    ).sin() * 0.5 + 0.5;

    let wave_pattern = (position.x * 15.0 + position.y * 15.0 + time).sin() * 0.5 + 0.5;
    
    // Mezcla de colores
    let mut final_color = color1.lerp(&color2, curve_pattern);
    final_color = final_color.lerp(&color3, wave_pattern * 0.7);

    if curve_pattern > 0.6 {
        final_color = final_color.lerp(&color4, curve_pattern - 0.3);
    } else if wave_pattern > 0.5 {
        final_color = final_color.lerp(&color5, wave_pattern - 0.3);
    }

    // Patrón
    let ring_pattern = uniforms.noise.get_noise_3d(
        position.x * 200.0 + time,
        position.y * 200.0,
        position.z * 200.0
    ).abs();

    if ring_pattern > 0.5 {
        final_color = final_color.lerp(&ring1_color, ring_pattern - 0.5);
    } else {
        final_color = final_color.lerp(&ring2_color, 0.5 - ring_pattern);
    }

    final_color * fragment.intensity
}


// Sol -- Estrella 
fn sun_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let position = fragment.vertex_position;
  let time = uniforms.time as f32 * 0.01;

  let core_color = Color::new(255, 200, 0);      // Amarillo brillante
  let corona_color = Color::new(255, 100, 0);    // Naranja para la corona
  
  // múltiples capas
  let plasma1 = uniforms.noise.get_noise_3d(
      position.x * 50.0 + time,
      position.y * 50.0,
      time * 2.0
  );
  
  let plasma2 = uniforms.noise.get_noise_3d(
      position.x * 30.0 - time,
      position.y * 30.0,
      time
  );
  
  // Corona solar 
  let corona = uniforms.noise.get_noise_3d(
      position.x * 10.0,
      position.y * 10.0,
      time * 0.5
  ).abs();
  
  // Combinar las capas
  let combined_noise = (plasma1 + plasma2) * 0.5;
  let final_color = core_color.lerp(&corona_color, combined_noise.abs());

  let brightness = 1.0 + corona * 0.5;
  
  final_color * brightness * fragment.intensity
}

// Planeta rocoso tipo Marte
fn rocky_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let position = fragment.vertex_position;
  let time = uniforms.time as f32 * 0.001;
  

  let desert_color = Color::new(180, 80, 20);     // Rojizo
  let crater_color = Color::new(120, 50, 10);     // Marrón oscuro
  let highland_color = Color::new(200, 100, 30);  // Naranja claro
  
  // Capa base de terreno
  let terrain = uniforms.noise.get_noise_3d(
      position.x * 100.0,
      position.y * 100.0,
      position.z * 100.0
  );
  
  // Capa de cráteres
  let craters = uniforms.noise.get_noise_3d(
      position.x * 200.0 + 1000.0,
      position.y * 200.0 + 1000.0,
      position.z * 200.0
  ).abs();
  
  // Capa de polvo moviéndose
  let dust = uniforms.noise.get_noise_3d(
      position.x * 50.0 + time,
      position.y * 50.0,
      position.z * 50.0
  );
  
  let mut final_color = desert_color;
  if craters > 0.7 {
      final_color = crater_color;
  } else if terrain > 0.3 {
      final_color = highland_color;
  }
  
  // "polvo"
  let dust_color = Color::new(200, 150, 100);
  final_color = final_color.lerp(&dust_color, dust.abs() * 0.3);
  
  final_color * fragment.intensity
}

fn gas_giant_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let position = fragment.vertex_position;
  let time = uniforms.time as f32 * 0.005;
  

  let band1_color = Color::new(255, 225, 190);  // Crema claro
  let band2_color = Color::new(210, 160, 110);  // Marrón claro
  let band3_color = Color::new(180, 130, 90);   // Marrón oscuro
  
  // Color para la Gran Mancha Roja y sus alrededores
  let storm_core_color = Color::new(255, 100, 80);  // Rojo intenso
  let storm_edge_color = Color::new(255, 140, 100); // Rojo suave para bordes

  let bands = uniforms.noise.get_noise_3d(
      position.x * 50.0 + time,
      position.y * 15.0 + time * 0.2,
      position.z * 50.0
  );

  // Generar un patrón 
  let secondary_bands = uniforms.noise.get_noise_3d(
      position.x * 25.0 + time * 0.5,
      position.y * 10.0 + time * 0.1,
      position.z * 25.0
  );

  // Mancha
  let storm = uniforms.noise.get_noise_3d(
      (position.x + 0.5) * 150.0,
      (position.y + 0.5) * 150.0,
      time
  ).abs();

  let turbulence = uniforms.noise.get_noise_3d(
      position.x * 100.0 + time * 2.0,
      position.y * 100.0,
      position.z * 100.0
  ).abs();

  // Determinar color base según la posición 
  let base_band_color = if bands > 0.2 {
      band1_color
  } else if secondary_bands > 0.0 {
      band2_color
  } else {
      band3_color
  };


  let mut final_color = if storm > 0.5 && position.x > 0.0 && position.y > 0.0 {
      storm_core_color.lerp(&storm_edge_color, (storm - 0.5) * 2.0)
  } else {
      base_band_color
  };

  final_color = final_color.lerp(&band3_color, turbulence * 0.3);

  final_color * fragment.intensity
}

// Planeta con nubes en movimiento
fn cloudy_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let position = fragment.vertex_position;
  let time = uniforms.time as f32 * 0.01;
  

  let surface_color = Color::new(30, 100, 200);  // Azul para océanos
  let land_color = Color::new(50, 120, 50);      // Verde para continentes
  let cloud_color = Color::new(255, 255, 255);   // Blanco para nubes
  
  let surface = uniforms.noise.get_noise_2d(
      position.x * 100.0,
      position.y * 100.0
  );
  
  // Capa de nubes con movimiento
  let clouds = uniforms.noise.get_noise_3d(
      position.x * 50.0 + time,
      position.y * 50.0 + time * 0.5,
      time
  );
  
  let base_color = if surface > 0.2 {
      land_color
  } else {
      surface_color
  };
  
  // Añadir nubes
  let final_color = if clouds > 0.3 {
      base_color.lerp(&cloud_color, (clouds - 0.3) * 2.0)
  } else {
      base_color
  };
  
  final_color * fragment.intensity
}

// Planeta con anillos 
fn ring_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let position = fragment.vertex_position;
  let time = uniforms.time as f32 * 0.001;
  
  let ring1_color = Color::new(180, 150, 120);  
  let ring2_color = Color::new(100, 80, 60);  
  
  // Patrón de anillos
  let ring_pattern = uniforms.noise.get_noise_3d(
      position.x * 200.0 + time,
      position.y * 200.0,
      position.z * 200.0
  );
  
  // Densidad variable en los anillos
  let density = uniforms.noise.get_noise_2d(
      position.x * 100.0,
      position.y * 100.0
  );
  
  // Combinar colores según el patrón
  let final_color = if ring_pattern > 0.0 {
      ring1_color.lerp(&ring2_color, density.abs())
  } else {
      ring2_color
  };
  
  let alpha = (density.abs() * 0.5 + 0.5) * fragment.intensity;
  final_color * alpha
}


fn ice_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let position = fragment.vertex_position;
  let time = uniforms.time as f32 * 0.002;
  

  let ice_color = Color::new(220, 240, 255);    
  let deep_ice_color = Color::new(150, 200, 255); 
  let crack_color = Color::new(100, 150, 255);   
  
  let ice_layers = uniforms.noise.get_noise_3d(
      position.x * 80.0,
      position.y * 80.0,
      position.z * 80.0 + time
  );
  
  // Grietas 
  let cracks = uniforms.noise.get_noise_3d(
      position.x * 120.0 + time,
      position.y * 120.0,
      position.z * 120.0
  ).abs();
  
  // Cristales de hielo
  let crystals = uniforms.noise.get_noise_3d(
      position.x * 200.0,
      position.y * 200.0,
      position.z * 200.0
  ).abs();
  
  let mut final_color = ice_color.lerp(&deep_ice_color, ice_layers.abs());
  
  // Aplicar grietas
  if cracks > 0.7 {
      final_color = final_color.lerp(&crack_color, (cracks - 0.7) * 2.0);
  }
  
  // Brillos de cristales
  if crystals > 0.8 {
      final_color = final_color.lerp(&ice_color, (crystals - 0.8) * 5.0);
  }
  
  final_color * fragment.intensity
}

fn moon_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let position = fragment.vertex_position;
  let time = uniforms.time as f32 * 0.001;

  let base_color = Color::new(180, 180, 180);  // Gris claro
  let crater_color = Color::new(100, 100, 100); // Gris oscuro
  let dust_color = Color::new(150, 150, 150);   // Gris medio

  // Patrón base de cráteres
  let craters = uniforms.noise.get_noise_3d(
      position.x * 150.0,
      position.y * 150.0,
      position.z * 150.0
  ).abs();

  // Patrón de polvo lunar
  let dust = uniforms.noise.get_noise_3d(
      position.x * 80.0 + time,
      position.y * 80.0,
      position.z * 80.0
  );

  // Detalles de la superficie
  let surface_details = uniforms.noise.get_noise_3d(
      position.x * 200.0,
      position.y * 200.0,
      position.z * 200.0
  ).abs();

  let mut final_color = base_color;

  // Aplicar cráteres
  if craters > 0.7 {
      final_color = final_color.lerp(&crater_color, (craters - 0.7) * 2.0);
  }

  // Aplicar polvo lunar
  final_color = final_color.lerp(&dust_color, dust.abs() * 0.2);

  // Añadir detalles de superficie
  if surface_details > 0.8 {
      final_color = final_color.lerp(&crater_color, (surface_details - 0.8) * 0.5);
  }

  final_color * fragment.intensity
}
