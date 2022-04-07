Feature: "init" command

  Scenario: no existing config file
    When initializing
    Then the exit code is 0
    And file "tikibase.json" should contain:
      """
      {
        "$schema": "https://raw.githubusercontent.com/kevgo/tikibase/main/doc/tikibase.schema.json"
      }
      """


  Scenario: existing config file
    Given file "tikibase.json" with content:
      """
      {
        foo: 1
      }
      """
    When initializing
    Then the exit code is 0
    And file "tikibase.json" should contain:
      """
      {
        "$schema": "https://raw.githubusercontent.com/kevgo/tikibase/main/doc/tikibase.schema.json"
      }
      """
