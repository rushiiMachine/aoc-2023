cp -R template day$1
sed -i "s/\$DAY/day$1/" day$1/Cargo.toml
sed -i -z "s/\n]/\n    \"day$1\",\n]/" Cargo.toml

echo "<component name=\"ProjectRunConfigurationManager\">
  <configuration default=\"false\" name=\"Run day$1\" type=\"CargoCommandRunConfiguration\" factoryName=\"Cargo Command\">
    <option name=\"command\" value=\"run --package day$1 --bin day$1\" />
    <option name=\"workingDirectory\" value=\"file://\$PROJECT_DIR$/day$1\" />
    <option name=\"emulateTerminal\" value=\"true\" />
    <option name=\"channel\" value=\"DEFAULT\" />
    <option name=\"requiredFeatures\" value=\"true\" />
    <option name=\"allFeatures\" value=\"false\" />
    <option name=\"withSudo\" value=\"false\" />
    <option name=\"buildTarget\" value=\"REMOTE\" />
    <option name=\"backtrace\" value=\"SHORT\" />
    <envs>
      <env name=\"RUST_BACKTRACE\" value=\"full\" />
    </envs>
    <option name=\"isRedirectInput\" value=\"false\" />
    <option name=\"redirectInputPath\" value=\"\" />
    <method v=\"2\">
      <option name=\"CARGO.BUILD_TASK_PROVIDER\" enabled=\"true\" />
    </method>
  </configuration>
</component>" > .idea/runConfigurations/Run_day$1.xml
