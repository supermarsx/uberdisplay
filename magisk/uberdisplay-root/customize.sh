#!/system/bin/sh
# Magisk module installer script.
SKIPMOUNT=false
PROPFILE=true
POSTFSDATA=false
LATESTARTSERVICE=true

print_modname() {
  ui_print "*******************************"
  ui_print "  UberDisplay Root Module"
  ui_print "*******************************"
}

on_install() {
  ui_print "- Installing UberDisplay root companion..."
  ui_print "- No system files are modified."
}

set_permissions() {
  set_perm_recursive "$MODPATH" 0 0 0755 0644
}
