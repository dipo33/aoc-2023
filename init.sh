#!/bin/bash

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 PACKAGE_NAME DISPLAY_NAME"
    exit 1
fi

PACKAGE_NAME=$1
DISPLAY_NAME=$2
TEMPLATE_DIR="./template"
CARGO_TOML_PATH="./Cargo.toml"
TMP_CARGO_TOML_PATH="./tmp.toml"

cp -r "$TEMPLATE_DIR" "$PACKAGE_NAME"

# Replace all placeholders
find "$PACKAGE_NAME" -type f -exec sed -i "s/%%PACKAGE_NAME%%/$PACKAGE_NAME/g" {} +
find "$PACKAGE_NAME" -type f -exec sed -i "s/%%DISPLAY_NAME%%/$DISPLAY_NAME/g" {} +

# Add the new project as dependency to the root cargo.toml
printf "\n" >> Cargo.toml
awk '
  BEGIN { in_deps = 0; printed = 0; }
  /\[dependencies\]/ { in_deps = 1; }
  !printed && in_deps && /^$/ { print "%%PACKAGE_NAME%% = { path = \"%%PACKAGE_NAME%%\" }"; printed = 1; }
  { print; }
' $CARGO_TOML_PATH > $TMP_CARGO_TOML_PATH
mv $TMP_CARGO_TOML_PATH $CARGO_TOML_PATH
sed -i '${/^$/d;}' $CARGO_TOML_PATH

# Add the new project as member of current workspace
awk '
  /members = \[/ { sub(/\]$/, ", \"%%PACKAGE_NAME%%\"]"); print; next; }
  { print; }
' $CARGO_TOML_PATH > $TMP_CARGO_TOML_PATH
mv $TMP_CARGO_TOML_PATH $CARGO_TOML_PATH

# Replace placeholders inside root Cargo.toml
sed -i "s/%%PACKAGE_NAME%%/$PACKAGE_NAME/g" $CARGO_TOML_PATH

echo "Project $PACKAGE_NAME created successfully"
