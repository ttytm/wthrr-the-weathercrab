use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

pub struct Graph(pub String, pub String);

struct GraphLvls {
	glyphs: Vec<char>,
	margin: f64,
	current: usize,
	next: usize,
	last: Option<usize>,
}

#[derive(Default, Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum GraphVariant {
	#[default]
	lines,
	lines_dotted,
	dotted,
}

impl Graph {
	pub fn prepare_graph(temperatures: &[f64], graph_variant: &GraphVariant) -> Result<Graph> {
		// TODO: use config variable
		let graph_rows = "double";
		let mut graph = Graph(String::new(), String::new());

		let min_temp = temperatures.iter().fold(f64::INFINITY, |a, &b| a.min(b));
		let max_temp = temperatures.iter().copied().fold(f64::NEG_INFINITY, f64::max);

		let mut graph_lvls = GraphLvls {
			glyphs: vec![' '],
			margin: 0.0,
			current: 0,
			next: 0,
			last: None,
		};
		graph_lvls.glyphs = GraphLvls::get_glyphs(graph_variant, graph_rows);
		graph_lvls.margin = (max_temp - min_temp) / (graph_lvls.glyphs.len() - 1) as f64;

		// Create Graph - calculate and push three characters per iteration to graph strings
		// Single Line Graph
		if graph_rows == "single" {
			for (i, temp) in temperatures.iter().enumerate() {
				graph_lvls.current = ((temp - min_temp) / graph_lvls.margin) as usize;
				graph_lvls.next = ((temperatures[i + 1] - min_temp) / graph_lvls.margin) as usize;

				// char 1/3 - compare with last level
				if let Some(last_lvl) = graph_lvls.last {
					match Some(last_lvl.cmp(&graph_lvls.current)) {
						Some(o) if o == Ordering::Less => graph.0.push(graph_lvls.glyphs[graph_lvls.get_idx_single(o)]),
						Some(o) if o == Ordering::Equal => {
							graph.0.push(graph_lvls.glyphs[graph_lvls.get_idx_single(o)])
						}
						Some(o) if o == Ordering::Greater => {
							graph.0.push(graph_lvls.glyphs[graph_lvls.get_idx_single(o)])
						}
						_ => {}
					}
				} else {
					// first iteration - without a last_lvl
					graph
						.0
						.push(graph_lvls.glyphs[graph_lvls.get_idx_single(Ordering::Equal)])
				}

				// char 2/3
				graph
					.0
					.push(graph_lvls.glyphs[graph_lvls.get_idx_single(Ordering::Equal)]);

				// char 3/3 - compare with next level
				match Some(graph_lvls.next.cmp(&graph_lvls.current)) {
					Some(o) if o == Ordering::Less => graph.0.push(graph_lvls.glyphs[graph_lvls.get_idx_single(o)]),
					Some(o) if o == Ordering::Equal => graph.0.push(graph_lvls.glyphs[graph_lvls.get_idx_single(o)]),
					Some(o) if o == Ordering::Greater => graph.0.push(graph_lvls.glyphs[graph_lvls.get_idx_single(o)]),
					_ => {}
				}

				if i == 23 {
					break;
				}

				graph_lvls.last = Some(graph_lvls.next);
			}

			return Ok(graph);
		}

		// Two Lines
		for (i, temp) in temperatures.iter().enumerate() {
			graph_lvls.current = ((temp - min_temp) / graph_lvls.margin) as usize;
			graph_lvls.next = ((temperatures[i + 1] - min_temp) / graph_lvls.margin) as usize;

			let graph_one_idx_sum = (graph_lvls.glyphs.len() - 1) / 2;

			// Char 1/3 - compare with last level
			if let Some(last_lvl) = graph_lvls.last {
				if graph_lvls.current > graph_one_idx_sum {
					match Some(last_lvl.cmp(&graph_lvls.current)) {
						Some(o) if o == Ordering::Less => {
							match graph_variant {
								GraphVariant::dotted => graph.0.push('⣿'),
								_ => graph.0.push(' '),
							}
							graph.1.push(graph_lvls.glyphs[graph_lvls.get_idx_double(o)]);
						}
						Some(o) if o == Ordering::Equal => {
							match graph_variant {
								GraphVariant::dotted => graph.0.push('⣿'),
								_ => graph.0.push(' '),
							}
							graph.1.push(graph_lvls.glyphs[graph_lvls.get_idx_double(o)]);
						}
						Some(o) if o == Ordering::Greater => {
							match graph_variant {
								GraphVariant::dotted => graph.0.push('⣿'),
								_ => graph.0.push(' '),
							}
							graph.1.push(graph_lvls.glyphs[graph_lvls.get_idx_double(o)]);
						}
						_ => {}
					}
				} else {
					match Some(last_lvl.cmp(&graph_lvls.current)) {
						Some(o) if o == Ordering::Less => {
							graph.0.push(graph_lvls.glyphs[graph_lvls.get_idx_double(o)]);
							graph.1.push(' ');
						}
						Some(o) if o == Ordering::Equal => {
							graph.0.push(graph_lvls.glyphs[graph_lvls.get_idx_double(o)]);
							graph.1.push(' ');
						}
						Some(o) if o == Ordering::Greater => {
							graph.0.push(graph_lvls.glyphs[graph_lvls.get_idx_double(o)]);
							graph.1.push(' ');
						}
						_ => {}
					}
				}
			} else {
				// First iteration - without a last level
				if graph_lvls.current > graph_one_idx_sum {
					match graph_variant {
						GraphVariant::dotted => graph.0.push('⣿'),
						_ => graph.0.push(' '),
					}
					graph
						.1
						.push(graph_lvls.glyphs[graph_lvls.get_idx_double(Ordering::Equal)]);
				} else {
					graph
						.0
						.push(graph_lvls.glyphs[graph_lvls.get_idx_double(Ordering::Equal)]);
					graph.1.push(' ');
				}
			}

			// Char 2/3
			if graph_lvls.current > graph_one_idx_sum {
				match graph_variant {
					GraphVariant::dotted => graph.0.push('⣿'),
					_ => graph.0.push(' '),
				}
				graph
					.1
					.push(graph_lvls.glyphs[graph_lvls.get_idx_double(Ordering::Equal)]);
			} else {
				graph
					.0
					.push(graph_lvls.glyphs[graph_lvls.get_idx_double(Ordering::Equal)]);
				graph.1.push(' ');
			}

			// Char 3/3 - compare with next level
			if graph_lvls.current > graph_one_idx_sum {
				match Some(graph_lvls.next.cmp(&graph_lvls.current)) {
					Some(o) if o == Ordering::Less => {
						match graph_variant {
							GraphVariant::dotted => graph.0.push('⣿'),
							_ => graph.0.push(' '),
						}
						graph.1.push(graph_lvls.glyphs[graph_lvls.get_idx_double(o)]);
					}
					Some(o) if o == Ordering::Equal => {
						match graph_variant {
							GraphVariant::dotted => graph.0.push('⣿'),
							_ => graph.0.push(' '),
						}
						graph.1.push(graph_lvls.glyphs[graph_lvls.get_idx_double(o)]);
					}
					Some(o) if o == Ordering::Greater => {
						match graph_variant {
							GraphVariant::dotted => graph.0.push('⣿'),
							_ => graph.0.push(' '),
						}
						graph.1.push(graph_lvls.glyphs[graph_lvls.get_idx_double(o)]);
					}
					_ => {}
				}
			} else {
				match Some(graph_lvls.next.cmp(&graph_lvls.current)) {
					Some(o) if o == Ordering::Less => {
						graph.0.push(graph_lvls.glyphs[graph_lvls.get_idx_double(o)]);
						graph.1.push(' ');
					}
					Some(o) if o == Ordering::Equal => {
						graph.0.push(graph_lvls.glyphs[graph_lvls.get_idx_double(o)]);
						graph.1.push(' ');
					}
					Some(o) if o == Ordering::Greater => {
						graph.0.push(graph_lvls.glyphs[graph_lvls.get_idx_double(o)]);
						graph.1.push(' ');
					}
					_ => {}
				}
			}

			if i == 23 {
				break;
			}

			let lvl_diff = graph_lvls.next as isize - graph_lvls.current as isize;

			graph_lvls.last = if lvl_diff.is_negative() && lvl_diff < -1 {
				Some(graph_lvls.current - 2)
			} else if lvl_diff.is_positive() && lvl_diff > 1 {
				Some(graph_lvls.current + 2)
			} else {
				Some(graph_lvls.next)
			};
		}

		Ok(graph)
	}
}

impl GraphLvls {
	fn get_glyphs(graph_variant: &GraphVariant, graph_rows: &str) -> Vec<char> {
		let mut glyphs = match graph_variant {
			GraphVariant::lines => vec!['▁', '🭻', '🭺', '🭹', '🭸', '🭷', '🭶', '▔'],
			GraphVariant::lines_dotted => vec!['⣀', '⠤', '⠒', '⠉'],
			GraphVariant::dotted => vec!['⣀', '⣤', '⣶', '⣿'],
		};

		if graph_rows == "double" {
			glyphs.append(&mut glyphs.to_vec())
		}

		glyphs
	}

	fn get_idx_single(&self, pending_comparison: Ordering) -> usize {
		match pending_comparison {
			Ordering::Less => {
				if self.next < self.current - 1 && self.current > 1 {
					self.current - 2
				} else {
					self.current - 1
				}
			}
			Ordering::Equal => {
				if self.next > self.current + 1 && self.current < self.glyphs.len() {
					self.current + 1
				} else if self.next < self.current && self.current > 0 {
					self.current - 1
				} else {
					self.current
				}
			}
			Ordering::Greater => {
				if self.next > self.current + 1 && self.current + 1 < self.glyphs.len() {
					self.current + 2
				} else {
					self.current + 1
				}
			}
		}
	}

	fn get_idx_double(&self, pending_comparison: Ordering) -> usize {
		let graph_one_max_idx = (self.glyphs.len() - 1) / 2;

		match pending_comparison {
			Ordering::Less => {
				if self.next < self.current - 1 && self.current > 1 {
					if self.current - 2 > graph_one_max_idx || self.current <= graph_one_max_idx {
						self.current - 2
					} else if self.current - 1 > graph_one_max_idx || self.current <= graph_one_max_idx {
						self.current - 1
					} else {
						self.current
					}
				} else {
					self.current
				}
			}
			Ordering::Equal => {
				if self.next > self.current + 1 && self.current < self.glyphs.len() {
					if self.current != graph_one_max_idx {
						self.current + 1
					} else {
						self.current
					}
				// this additional clause should further improve details, but small deviations make the graph look a bit scattered
				/* } else if self.next < self.current && self.current > 0 {
				if self.current - 1 > graph_one_idx_sum || self.current <= graph_one_idx_sum {
					self.current - 1
				} else {
					self.current
				} */
				} else {
					self.current
				}
			}
			Ordering::Greater => {
				if self.next > self.current + 1 && self.current + 1 < self.glyphs.len() {
					if self.current + 2 <= graph_one_max_idx || self.current > graph_one_max_idx {
						self.current + 2
					} else if self.current != graph_one_max_idx {
						self.current + 1
					} else {
						self.current
					}
				} else if self.current != graph_one_max_idx {
					self.current + 1
				} else {
					self.current
				}
			}
		}
	}
}
