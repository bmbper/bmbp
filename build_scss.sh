for dir in crates/bmbp_*/; do
  echo "找到模块: $dir"
  if [ -d "${dir}web/scss" ]; then
    echo "编译样式文件......"
    find "${dir}web/scss" -type f -name "index.scss" | while read -r scss_file; do
      # 获取 index.scss 所在的目录路径，相对于 web/scss
      relative_dir=$(dirname "${scss_file#${dir}web/scss/}")
      echo "编译 $relative_dir"
      echo "输出 $dir"
      module_name=${dir#crates/}
      # 创建输出目录结构
      output_dir="${dir}/static/${module_name}/css/${relative_dir}"
      echo "输出目录 $output_dir"
      mkdir -p "$output_dir"
      # 使用 sass 命令编译 .scss 文件为 .css 文件
      sass "$scss_file" "$output_dir/index.css"
      echo "编译样式完成 $scss_file"
      echo "编译完成， 输出到 $output_dir/index.css"
    done
  else
    echo "$dir 不存在样式文件夹，跳过"
  fi
done