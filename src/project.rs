use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::fs::File;
use std::str::FromStr;
use std::error::Error;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashMap;

const PROJECTS_FILE: &str = "projects.json";

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ProjectTaskType {
	Default,
}

impl FromStr for ProjectTaskType {
	type Err = ();

	fn from_str(input: &str) -> Result<ProjectTaskType, Self::Err> {
		match input {
			"default" => Ok(ProjectTaskType::Default),
			_         => Err(()),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ProjectTaskStatus {
	Default,
	Todo,
	InProgress,
	Complete,
}

impl FromStr for ProjectTaskStatus {
	type Err = ();

	fn from_str(input: &str) -> Result<ProjectTaskStatus, Self::Err> {
		match input {
			"todo"       => Ok(ProjectTaskStatus::Todo),
			"in_progress" => Ok(ProjectTaskStatus::InProgress),
			"complete"   => Ok(ProjectTaskStatus::Complete),
			_            => Ok(ProjectTaskStatus::Default),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectTask {
	pub id: Uuid,
	pub name: String,
	pub description: String,
	pub type_: ProjectTaskType,
	pub status: ProjectTaskStatus,
}

impl ProjectTask {
	pub fn new(name: &str, description: &str, type_: &str, status: &str) -> Self {
		let task_type_result = ProjectTaskType::from_str(type_);
		let task_type = match task_type_result {
			Ok(task_type) => task_type,
			Err(_) => ProjectTaskType::Default,
		};
		let task_status_result = ProjectTaskStatus::from_str(status);
		let task_status = match task_status_result {
			Ok(task_status) => task_status,
			Err(_) => ProjectTaskStatus::Todo,
		};

		Self {
			id: Uuid::new_v4(),
			name: name.to_string(),
			description: description.to_string(),
			type_: task_type,
			status: task_status,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Project {
	pub id: Uuid,
	pub name: String,
	pub description: String,
	pub tasks: HashMap<Uuid, ProjectTask>,
}

impl Project {
	pub fn new(name: &String, description: &String) -> Self {
		Self {
			id: Uuid::new_v4(),
			name: name.to_string(),
			description: description.to_string(),
			tasks: HashMap::new(),
		}
	}

	pub fn create_task(&mut self, name: &String, description: &String) -> Uuid {
		let task = ProjectTask::new(name, description, "default", "todo");
		let task_id = task.id.clone();

		self.tasks.insert(task_id, task);

		task_id
	}

	pub fn destroy_task(&mut self, task_id: &Uuid) -> Result<bool, Box<dyn Error>> {
		self.tasks.remove(task_id).unwrap();

		Ok(true)
	}
}

#[derive(Debug, Clone)]
pub struct ProjectData {
	projects: HashMap<Uuid, Project>,
}

impl ProjectData {
	pub fn create_project(&mut self, name: &String, description: &String) -> Uuid {
		let project = Project::new(name, description);
		let project_id = project.id.clone();
		self.projects.insert(project_id, project);

		project_id
	}

	pub fn destroy_project(&mut self, project_id: &Uuid) -> Result<bool, Box<dyn Error>> {
		self.projects.remove(project_id).unwrap();

		Ok(true)
	}

	pub fn get_project(&self, project_id: &Uuid) -> Option<&Project> {
		self.projects.get(project_id)
	}

	pub fn get_project_mut(&mut self, project_id: &Uuid) -> Option<&mut Project> {
		self.projects.get_mut(project_id)
	}

	pub fn get_projects(&self) -> Vec<&Project> {
		self.projects.values().collect()
	}
}

fn load_projects() -> Result<HashMap<Uuid, Project>, Box<dyn Error>> {
	if !Path::new(PROJECTS_FILE).exists() {
        return Ok(HashMap::new());
    }
	let file = File::open(PROJECTS_FILE)?;
    let reader = BufReader::new(file);
    let projects = serde_json::from_reader(reader)?;

	Ok(projects)
}

pub fn load_data() -> Result<ProjectData, Box<dyn Error>> {
	let projects = load_projects()?;

	Ok(ProjectData { projects })
}

pub fn write_data(data: &ProjectData) -> Result<(), Box<dyn Error>> {
	let file = File::create(PROJECTS_FILE)?;
	serde_json::to_writer(file, &data.projects)?;

	Ok(())
}