use std::cmp;
use std::collections::LinkedList;
fn main() {}

fn ford_fulkerson(graph: Vec<Vec<usize>>, V: usize, s: usize, t: usize) -> usize {
    let mut u: isize = 0;
    let mut v: usize = 0;
    let mut rGraph: Vec<Vec<usize>> = Vec::new();

    for i in 0..V {
        u += 1;
        for k in 0..V {
            rGraph[i][k] = graph[i][k];
            v += 1;
        }
    }

    let mut parent: Vec<isize> = Vec::new();
    let mut max_flow = 0;

    while bfs(&rGraph, s, t, &mut parent, V) {
        let mut path_flow = usize::max_value();

        v = t.clone();
        while v != s {
            u = parent[v].clone();
            path_flow = cmp::min(path_flow, rGraph[u as usize][v as usize]);

            v = parent[v as usize].clone() as usize;
        }

        v = t.clone();
        while v != s {
            u = parent[v].clone();
            rGraph[u as usize][v] -= path_flow;
            rGraph[v][u as usize] += path_flow;

            v = parent[v].clone() as usize;
        }

        max_flow += path_flow;
    }

    return max_flow;
}

fn bfs(rGraph: &Vec<Vec<usize>>, s: usize, t: usize, parent: &mut Vec<isize>, V: usize) -> bool {
    let mut visited: Vec<bool> = Vec::new();

    for _ in 0..V {
        visited.push(false);
    }

    let mut queue: Vec<usize> = Vec::new();
    queue.push(s);
    visited[s] = true;
    parent[s] = -1;

    while queue.len() != 0 {
        let u = queue.remove(0);
        for v in 0..V {
            if visited[v] == false && rGraph[u][v] > 0 {
                queue.push(v);
                parent[v] = u as isize;
                visited[v] = true;
            }
        }
    }

    return visited[t] == true;
}
