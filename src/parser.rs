pub struct Material {
    name: String,
    ambiant_reflectivity: [f32; 3],
    diffuse_reflectivity: [f32; 3],
    specular_reflectivity: [f32; 3],
    illumination: i32,
    factor: f32,
    specular_exponent: f32,
    optical_density: f32,
}

impl Material{
    pub fn default() -> Self {
        Material {
            name: "".to_string(),
            ambiant_reflectivity: [-1.0, -1.0, -1.0],
            diffuse_reflectivity: [-1.0, -1.0, -1.0],
            specular_reflectivity: [-1.0, -1.0, -1.0],
            illumination: -1,
            factor: -1.0,
            specular_exponent: -1.0,
            optical_density: -1.0,
        }
    }
}

pub struct Index {
    vertices: i32,
    textures: i32,
    normals: i32,
}

pub struct Data {
    name: String,
    vertices: Vec<[f32; 3]>,
    textures_coord: Vec<[f32; 2]>,
    normals: Vec<[f32; 3]>,
    links: (Vec<Vec<Index>>, Material)
}

impl Data {
    pub fn default() -> Self{
        Data {
            name: "".to_string(),
            vertices: Vec::new(),
            textures_coord: Vec::new(),
            normals: Vec::new(),
            links: (Vec::new(), Material::default()),
        }
    }
   
    fn parse_name(&mut self, line: &String) {
        let line_name : Vec<&str> = line.split_whitespace().collect();
    
        if line_name.len() != 2 {
            panic!("error at line: {line}");
        }
        self.name = line_name[1].to_string();
    }
    
    fn parse_vertex(&mut self, line: &String) {
        let line_vertex : Vec<&str> = line.split_whitespace().collect();
    
        if line_vertex.len() != 4 {
            panic!("error at line: {line}");
        }
        
        self.vertices.push([
            line_vertex[1].parse().expect("Error in .obj file"),
            line_vertex[2].parse().expect("Error in .obj file"),
            line_vertex[3].parse().expect("Error in .obj file"),
        ]);
    }

    fn parse_normals(&mut self, line: &String) {
        let line_vertex : Vec<&str> = line.split_whitespace().collect();
    
        if line_vertex.len() != 4 {
            panic!("error at line: {line}");
        }
        self.normals.push([
            line_vertex[1].parse().expect("Error in .obj file"),
            line_vertex[2].parse().expect("Error in .obj file"),
            line_vertex[3].parse().expect("Error in .obj file"),
        ]);
    }

    fn parse_textures(&mut self, line: &String) {
        let line_textures : Vec<&str> = line.split_whitespace().collect();
    
        if line_textures.len() != 3 {
            panic!("error at line: {line}");
        }
        self.textures_coord.push([
            line_textures[1].parse().expect("Error in .obj file"),
            line_textures[2].parse().expect("Error in .obj file"),
        ]);
    }

    fn parse_links(&mut self, line: &str, material: Material) {
        let line_links : Vec<&str> = line.split_whitespace().collect();

        if line_links.len() <= 2 {
            panic!("error at line: {line}");
        }

        let mut index_vec: Vec<Index> = Vec::new();
        for elem in line_links {
            let nb_slash = elem.matches('/').count();
            if nb_slash == 0 {
                index_vec.push(Index{
                    vertices: elem.parse().expect("Error in .obj file"),
                    textures: -1,
                    normals: -1,
                })
            }
            else if nb_slash == 1 {
                let vert_text: Vec<&str> = elem.split("/").collect();
                index_vec.push(Index{
                    vertices: vert_text[0].parse().expect("Error in .obj file"),
                    textures: vert_text[1].parse().expect("Error in .obj file"),
                    normals: -1,
                })
            }
            else if nb_slash == 2 && elem.contains("//") {
                let vert_normals: Vec<&str> = elem.split("/").collect();
                index_vec.push(Index{
                    vertices: vert_normals[0].parse().expect("Error in .obj file"),
                    textures: -1,
                    normals: vert_normals[1].parse().expect("Error in .obj file"),
                })
            }
            else if nb_slash == 2 {
                let all: Vec<&str> = elem.split("/").collect();
                index_vec.push(Index{
                    vertices: all[0].parse().expect("Error in .obj file"),
                    textures: all[1].parse().expect("Error in .obj file"),
                    normals: all[2].parse().expect("Error in .obj file"),
                })
            }
        }
        self.links.0.push(index_vec);
        self.links.1 = material;
    }

    pub fn line_type_choice(&mut self, line: &String) {
        let current_material = Material::default();
        
        match line.as_str() {
            line if line.starts_with('#') => {},
            line if line.starts_with('o') => self.parse_name(&line.to_string()),
            line if line.starts_with('v') => self.parse_vertex(&line.to_string()),
            line if line.starts_with('f') => self.parse_links(&line.to_string(), current_material),
            line if line.starts_with("vt") => self.parse_textures(&line.to_string()),
            line if line.starts_with("vn") => self.parse_normals(&line.to_string()),
            line if line.starts_with("mtllib")  => {println!("mttlib")},
            line if line.starts_with("usemtl")  => {println!("usemtl")},
            line if line.starts_with('s')  => {println!("s")},
            _ => panic!("Error in the .obj file at line '{line}'"),
        };
    }

}