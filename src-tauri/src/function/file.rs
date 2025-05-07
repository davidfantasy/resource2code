use anyhow::{anyhow, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub fn file_existed(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn save_file(path: &str, content: &str) -> Result<bool> {
    let file_path = PathBuf::from(path);
    if file_path.exists() {
        fs::write(&file_path, content)
            .map_err(|e| anyhow!("修改文件失败: {}, 错误: {}", &file_path.display(), e))?;
        Ok(false)
    } else {
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| anyhow!("创建目录失败: {}, 错误: {}", parent.display(), e))?;
        }
        fs::write(&file_path, content)
            .map_err(|e| anyhow!("创建文件失败: {}, 错误: {}", file_path.display(), e))?;
        Ok(true)
    }
}

pub fn merge_paths(root_path: &str, sub_path: &str) -> PathBuf {
    let root = Path::new(root_path);
    let sub = Path::new(sub_path);

    // 收集 Normal 类型的组件
    let root_comps: Vec<String> = get_normal_components(root);
    let sub_comps: Vec<String> = get_normal_components(sub);

    // 找出最大公共前缀长度
    let n = find_max_common_suffix(&root_comps, &sub_comps);

    // 创建一个新的 PathBuf，从 root_path 开始
    let mut result = PathBuf::from(root_path);

    // 添加 sub_path 中未被去重的部分
    for part in &sub_comps[n..] {
        result.push(part);
    }

    result
}

// 提取 Normal 类型的路径组件（忽略 CurDir、ParentDir、RootDir 等）
fn get_normal_components(path: &Path) -> Vec<String> {
    path.components()
        .filter_map(|comp| match comp {
            std::path::Component::Normal(os_str) => os_str.to_str().map(String::from),
            _ => None,
        })
        .collect()
}

// 找出最大的公共后缀/前缀长度
fn find_max_common_suffix(root_comps: &[String], sub_comps: &[String]) -> usize {
    let max_possible = std::cmp::min(root_comps.len(), sub_comps.len());
    for n in (0..=max_possible).rev() {
        if root_comps.ends_with(&sub_comps[..n]) {
            return n;
        }
    }
    0
}
