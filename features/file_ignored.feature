Feature: Ignore files

  Background:
    Given file "tikibase.json" with content:
      """
      {
        "ignore": [
          "Makefile",
        ]
      }
      """
    And binary file "Makefile"
