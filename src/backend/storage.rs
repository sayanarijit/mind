use crate::backend::horizon::HorizonPriorityImpl;
use crate::backend::information::InformationImpl;
use crate::backend::label::InformationLabelRelationImpl;
use crate::backend::label::LabelImpl;
use crate::object::Object;
use anyhow::Error;
use anyhow::Result;
use std::fmt::Debug;
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

fn get_storage(base: &PathBuf, dir: &str) -> Result<PathBuf> {
    let path = if !dir.is_empty() {
        base.join(dir)
    } else {
        base.clone()
    };

    if !path.exists() {
        fs::create_dir_all(&path)?;
    }

    Ok(path)
}

fn path_to_id(path: PathBuf) -> Option<Uuid> {
    path.file_name()
        .and_then(|n| n.to_str().and_then(|s| Uuid::parse_str(s).ok()))
}

pub struct Storage {
    path: PathBuf,
    information_path: PathBuf,
    label_path: PathBuf,
    information_label_relation_path: PathBuf,
    horizon_priority_path: PathBuf,
}

impl Storage {
    fn list_objects<T>(
        path: &PathBuf,
        transform: impl Fn(&Uuid) -> Result<T>,
    ) -> Result<Vec<T>> {
        let res: Vec<T> = fs::read_dir(path)?
            .filter_map(|d| d.ok().map(|p| p.path()))
            .filter_map(path_to_id)
            .filter_map(|p| transform(&p).ok())
            .collect::<Vec<T>>();
        Ok(res)
    }

    fn get_object<T: Debug>(
        path: &PathBuf,
        id: &Uuid,
        transform: impl Fn(&str) -> Result<T>,
    ) -> Result<T> {
        let path = path.join(id.to_string());
        let string = fs::read_to_string(&path)?;
        let res = transform(&string)?;
        Ok(res)
    }

    fn save_object<T: Object<Id = I>, I: Display>(
        path: &PathBuf,
        object: &T,
        transform: impl Fn(&T) -> Result<String>,
    ) -> Result<()> {
        let res = transform(object)?;
        fs::write(path.join(object.id().to_string()), res)?;
        Ok(())
    }

    fn delete_object<T: Object<Id = I>, I: Display>(
        path: &PathBuf,
        object: T,
    ) -> Result<()> {
        fs::remove_file(path.join(object.id().to_string()))?;
        Ok(())
    }

    pub fn new(path: PathBuf) -> Result<Self> {
        let path = get_storage(&path, "")?;
        let information_path = get_storage(&path, "information")?;
        let label_path = get_storage(&path, "label")?;
        let information_label_relation_path =
            get_storage(&path, "many_information_many_label")?;
        let horizon_priority_path = get_storage(&path, "horizon_priority")?;

        let storage = Self {
            path: path.clone(),
            information_path,
            label_path,
            information_label_relation_path,
            horizon_priority_path,
        };

        Ok(storage)
    }

    /// Get a reference to the storage's path.
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn get_horizon_priority(
        &self,
        id: &Uuid,
    ) -> Result<HorizonPriorityImpl> {
        Self::get_object(&self.horizon_priority_path, id, |s| {
            toml::from_str(s).map_err(Error::from)
        })
    }

    // TODO: Use macro for the boilerplate functions

    pub fn list_horizon_priority(&self) -> Result<Vec<HorizonPriorityImpl>> {
        Self::list_objects(&self.horizon_priority_path, |p| {
            self.get_horizon_priority(p)
        })
    }

    pub fn save_horizon_priority(
        &self,
        object: &HorizonPriorityImpl,
    ) -> Result<()> {
        Self::save_object(&self.horizon_priority_path, object, |o| {
            toml::to_string_pretty(o).map_err(Error::from)
        })
    }

    pub fn delete_horizon_priority(
        &self,
        object: HorizonPriorityImpl,
    ) -> Result<()> {
        Self::delete_object(&self.horizon_priority_path, object)
    }

    pub fn get_information(&self, id: &Uuid) -> Result<InformationImpl> {
        let path = self.information_path.join(id.to_string());
        let res = toml::from_str(&fs::read_to_string(path)?)?;
        Ok(res)
    }

    pub fn list_information(&self) -> Result<Vec<InformationImpl>> {
        Self::list_objects(&self.information_path, |p| self.get_information(p))
    }

    pub fn save_information(&self, object: &InformationImpl) -> Result<()> {
        Self::save_object(&self.information_path, object, |o| {
            toml::to_string_pretty(o).map_err(Error::from)
        })
    }

    pub fn delete_information(
        &self,
        object: HorizonPriorityImpl,
    ) -> Result<()> {
        Self::delete_object(&self.information_path, object)
    }

    pub fn get_label(&self, id: &Uuid) -> Result<LabelImpl> {
        Self::get_object(&self.label_path, id, |s| {
            toml::from_str(s).map_err(Error::from)
        })
    }

    pub fn list_labels(&self) -> Result<Vec<LabelImpl>> {
        Self::list_objects(&self.label_path, |p| self.get_label(p))
    }

    pub fn save_label(&self, object: &LabelImpl) -> Result<()> {
        Self::save_object(&self.label_path, object, |o| {
            toml::to_string_pretty(o).map_err(Error::from)
        })
    }

    pub fn delete_label(&self, object: HorizonPriorityImpl) -> Result<()> {
        Self::delete_object(&self.label_path, object)
    }

    pub fn get_information_label_relation(
        &self,
        id: &Uuid,
    ) -> Result<InformationLabelRelationImpl<InformationImpl, LabelImpl>> {
        Self::get_object(&self.information_label_relation_path, id, |s| {
            toml::from_str(s).map_err(Error::from)
        })
    }

    pub fn list_information_label_relation(
        &self,
    ) -> Result<Vec<InformationLabelRelationImpl<InformationImpl, LabelImpl>>>
    {
        Self::list_objects(&self.information_label_relation_path, |p| {
            self.get_information_label_relation(p)
        })
    }

    pub fn save_information_label_relation(
        &self,
        object: &InformationImpl,
    ) -> Result<()> {
        Self::save_object(&self.information_label_relation_path, object, |o| {
            toml::to_string_pretty(o).map_err(Error::from)
        })
    }

    pub fn delete_information_label_relation(
        &self,
        object: InformationLabelRelationImpl<InformationImpl, LabelImpl>,
    ) -> Result<()> {
        Self::delete_object(&self.information_label_relation_path, object)
    }
}
