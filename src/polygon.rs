use crate::ext::string::StringExtension;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct Vertex {
    position: usize,
    normal: Option<usize>,
    texture: Option<usize>,
}

impl std::fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.position)
    }
}

// //valid line: `f 16774//19105 16424//19053 16242//19104 16552//19106`
// 		public static bool ReadPolygonData(string line, List<string> polygonVertices)
// 		{
//
//
// 			int currentIndex = line.IndexOf(SpaceSeparator);
// 			while (currentIndex != -1)
// 			{
// 				int nextIndex = line.IndexOf(SpaceSeparator, currentIndex + 1);
// 				int substringLength = nextIndex < 0 ? line.Length - currentIndex - 1 : nextIndex - currentIndex - 1;
// 				polygonVertices.Add(line.Substring(currentIndex + 1, substringLength));
//
// 				currentIndex = nextIndex;
// 			}
//
// 			return polygonVertices.Count > 2;
// 		}
//
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Polygon {
    vertices: Vec<String>,
}

impl Polygon {
    pub fn from_string(polygon_string: &String) -> Self {
        let mut vertices: Vec<String> = vec![];

        let mut current_index = polygon_string.find(" ").unwrap();
        while current_index != 0 {
            let next_index = match polygon_string.find_start_at(" ", current_index + 1) {
                Some(n) => n,
                None => 0,
            };

            let substring_length = if next_index == 0 {
                polygon_string.len() - current_index
            } else {
                next_index - current_index
            };
            vertices.push(
                polygon_string[current_index + 1..current_index + substring_length].to_string(),
            );
            current_index = next_index;
        }

        Self { vertices: vertices }
    }
}

impl std::fmt::Display for Polygon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Polygon: ").unwrap();
        for vertex in &self.vertices {
            write!(f, "{}, ", vertex).unwrap();
        }

        Ok(())
    }
}
