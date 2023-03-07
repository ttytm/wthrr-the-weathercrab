use optional_struct::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[optional_struct(ConfigFileGraphOpts)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct GraphOpts {
	pub style: GraphStyle,
	pub rowspan: GraphRows,
	pub time_indicator: bool,
}

impl Default for GraphOpts {
	fn default() -> Self {
		Self {
			style: GraphStyle::default(),
			rowspan: GraphRows::default(),
			time_indicator: true,
		}
	}
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum GraphStyle {
	lines(LineVariant),
	dotted,
	custom([char; 8]),
}

#[derive(Default, Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum LineVariant {
	slim,
	#[default]
	solid,
	dotted,
}

impl Default for GraphStyle {
	fn default() -> Self {
		Self::lines(LineVariant::default())
	}
}

#[derive(Default, Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum GraphRows {
	single,
	#[default]
	double,
}

pub struct Graph(pub String, pub String);

struct GraphLvls {
	glyphs: Vec<char>,
	margin: f32,
	current: usize,
	next: usize,
	last: Option<usize>,
}

impl Graph {
	pub fn prepare_graph(temperatures: &[f32], graph_opts: &GraphOpts) -> Graph {
		let mut graph = Graph(String::new(), String::new());
		let style = graph_opts.style;
		let rowspan = graph_opts.rowspan;

		let min_temp = temperatures.iter().fold(f32::INFINITY, |a, &b| a.min(b));
		let max_temp = temperatures.iter().copied().fold(f32::NEG_INFINITY, f32::max);

		let mut graph_lvls = GraphLvls {
			glyphs: vec![' '],
			margin: 0.0,
			current: 0,
			next: 0,
			last: None,
		};
		graph_lvls.glyphs = GraphLvls::get_glyphs(&style, &rowspan);
		graph_lvls.margin = (max_temp - min_temp) / (graph_lvls.glyphs.len() - 1) as f32;

		// Create Graph - calculate and push three characters per iteration to graph strings
		// Single Line Graph
		if rowspan == GraphRows::single {
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
					graph.0.push(graph_lvls.glyphs[graph_lvls.get_idx_single(Ordering::Equal)])
				}

				// char 2/3
				graph.0.push(graph_lvls.glyphs[graph_lvls.get_idx_single(Ordering::Equal)]);

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

			return graph;
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
							match style {
								GraphStyle::dotted => graph.0.push('⣿'),
								_ => graph.0.push(' '),
							}
							graph.1.push(graph_lvls.glyphs[graph_lvls.get_idx_double(o)]);
						}
						Some(o) if o == Ordering::Equal => {
							match style {
								GraphStyle::dotted => graph.0.push('⣿'),
								_ => graph.0.push(' '),
							}
							graph.1.push(graph_lvls.glyphs[graph_lvls.get_idx_double(o)]);
						}
						Some(o) if o == Ordering::Greater => {
							match style {
								GraphStyle::dotted => graph.0.push('⣿'),
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
					match style {
						GraphStyle::dotted => graph.0.push('⣿'),
						_ => graph.0.push(' '),
					}
					graph.1.push(graph_lvls.glyphs[graph_lvls.get_idx_double(Ordering::Equal)]);
				} else {
					graph.0.push(graph_lvls.glyphs[graph_lvls.get_idx_double(Ordering::Equal)]);
					graph.1.push(' ');
				}
			}

			// Char 2/3
			if graph_lvls.current > graph_one_idx_sum {
				match style {
					GraphStyle::dotted => graph.0.push('⣿'),
					_ => graph.0.push(' '),
				}
				graph.1.push(graph_lvls.glyphs[graph_lvls.get_idx_double(Ordering::Equal)]);
			} else {
				graph.0.push(graph_lvls.glyphs[graph_lvls.get_idx_double(Ordering::Equal)]);
				graph.1.push(' ');
			}

			// Char 3/3 - compare with next level
			if graph_lvls.current > graph_one_idx_sum {
				match Some(graph_lvls.next.cmp(&graph_lvls.current)) {
					Some(o) if o == Ordering::Less => {
						match style {
							GraphStyle::dotted => graph.0.push('⣿'),
							_ => graph.0.push(' '),
						}
						graph.1.push(graph_lvls.glyphs[graph_lvls.get_idx_double(o)]);
					}
					Some(o) if o == Ordering::Equal => {
						match style {
							GraphStyle::dotted => graph.0.push('⣿'),
							_ => graph.0.push(' '),
						}
						graph.1.push(graph_lvls.glyphs[graph_lvls.get_idx_double(o)]);
					}
					Some(o) if o == Ordering::Greater => {
						match style {
							GraphStyle::dotted => graph.0.push('⣿'),
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

		graph
	}
}

impl GraphLvls {
	fn get_glyphs(graph_style: &GraphStyle, graph_rows: &GraphRows) -> Vec<char> {
		let mut glyphs = match graph_style {
			GraphStyle::lines(style) => match style {
				LineVariant::solid => vec!['▁', '🭻', '🭺', '🭹', '🭸', '🭷', '🭶', '▔'],
				LineVariant::slim => ['⎽', '⎼', '⎻', '⎺'].to_vec(),
				LineVariant::dotted => ['⣀', '⠤', '⠒', '⠉'].to_vec(),
			},
			GraphStyle::dotted => vec!['⣀', '⣤', '⣶', '⣿'],
			GraphStyle::custom(vec) => {
				let mut custom_glyphs = vec![];
				for &char in vec {
					custom_glyphs.push(char);
				}
				custom_glyphs
			}
		};

		if graph_rows == &GraphRows::double {
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
				if self.current - 1 > graph_one_max_idx || self.current <= graph_one_max_idx {
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
