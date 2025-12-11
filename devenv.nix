{
  pkgs,
  ...
}:

{
  packages = [ pkgs.git pkgs.openssl ];
  languages.rust.enable = true;
}
