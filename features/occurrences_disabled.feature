Feature: disabled Bidi links

  Background:
    Given file "1.md" with content:
      """
      # One

      ### section
      [Two](2.md)

      ### occurrences
      text
      """
    And file "2.md" with content:
      """
      # Other
      [One](1.md)
      """
    And file "tikibase.json" with content:
      """
      {
        "bidiLinks": false
      }
      """

  Scenario: check
    When checking
    Then it finds no issues
    And the exit code is 0

  Scenario: fix
    When fixing
    Then it finds no issues
    And the exit code is 0

  Scenario: pitstop
    When doing a pitstop
    Then it finds no issues
    And the exit code is 0
