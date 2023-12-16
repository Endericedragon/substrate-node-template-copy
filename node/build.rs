use substrate_build_script_utils::{generate_cargo_keys, rerun_if_git_head_changed};

fn main() {
	// 获取并打印一些版本信息
	generate_cargo_keys();
	// 从$CARGO_MANIFEST_DIR开始，逐层向上检查git仓库目录
	rerun_if_git_head_changed();
}
