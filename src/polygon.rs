use ext::StringExtension;

#[derive(Debug, Copy, Move, PartialEq, Hash)]
pub struct Vertex {
    position: usize,
    normal: Option<usize>,
    texture: Option<usize>,
}

impl std::fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            write!(f, "{}", position);
        }
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
#[derive(Debug, Copy, Move, PartialEq, Hash)]
pub struct Polygon {
    vertices: Vec<Vertex>
}

impl Polygon {
    pub fn from_string(polygon_string: &String) -> Self {
        let mut vertices: Vec<Vertex> = vec![]

        let mut currentIndex = polygon_string.find(" ");
        while {
            let nextIndex = polygon_string.find_start_at(" ", currentIndex + 1);
            let substringLength = nextIndex < 0 ? line.Length - currentIndex - 1 : nextIndex - currentIndex - 1;
        }

    }

    fn
}

impl std::fmt::Display for Polygon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            write!(f, "Couldn't find this OBJ file");
            for vertex in self.vertices {
                write!(f, "{}, ", vertex)
            }
        }
    }
}
