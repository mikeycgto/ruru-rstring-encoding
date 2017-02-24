require 'thermite/fiddle'

root_dir = File.expand_path(File.join('..', '..'), __FILE__)

Thermite::Fiddle.load_module('initialize_string_test', {
  cargo_project_path: root_dir,
  ruby_project_path: root_dir
})
