// make a graph struct with vertice vector & edge vector(like pair in vector). create impl for functions to set these

use std::collections::HashSet;

struct Graph{
    vertices: Vec<i32>,
    edges: HashSet<(i32,i32)>
}

impl Graph{

    fn new() -> Self{
        Self { vertices: Vec::new(), edges: HashSet::new() }
    }

    fn set_vertices(&mut self,n:i32){
        for i in 1..=n{
        self.vertices.push(i);}
    }
    fn return_edges(&self) -> HashSet<(i32,i32)>{
       self.edges.clone()
    }
    fn return_vertices(&self) -> Vec<i32>{
        self.vertices.clone()
    }
    fn return_n(&self) -> i32{
        self.vertices.len() as i32
    }
}

fn validate_degree(d: i32,g: &Graph) -> bool{
    g.return_vertices().len() - 1  >= d.try_into().unwrap()
}

fn make_edges(d: i32, g: &mut Graph) -> Option<HashSet<(i32,i32)>>{
    if !(validate_degree(d, g)){
        None
    }else
    {
        let tn = g.return_n() as usize;
        let verts = g.return_vertices();
        for i in 0..=tn-2{
            for j in 1..=d as usize{
                let (a,b)= (verts[i],verts[(i+j)%tn]);
                let edge = if a < b { (a, b) } else { (b, a) };
                g.edges.insert(edge);
                }
        }
        Some(g.return_edges())
    }
}


fn main(){
    let mut t = Graph::new();
    t.set_vertices(4);
    let mut e = make_edges(2, &mut t);
    e = match e{
        Some(k) => Some(k),
        None => {println!("Not a valid degree for the graph");None}
    };
    if e.is_some(){
        println!("{:?}",t.return_edges());
    }else{
        return;
    }
    

}
