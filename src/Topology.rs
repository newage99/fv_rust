use super::Graph::Graph;
use super::FVID::FVID;

pub struct Topology {
    pub fvid: FVID,
    pub graphs: Vec<Graph>
}

impl Topology {

    pub fn total_degree(&self) -> i128 {

        let mut degree: i128 = 0;

        for graph in &self.graphs {
            degree += graph.degree();
        }

        degree
    }

    pub fn is_connected(&self) -> bool {
        let mut connected: bool = true;
        for graph in &self.graphs {
            if !graph.is_connected() {
                connected = false;
            }
        }
        connected
    }
}