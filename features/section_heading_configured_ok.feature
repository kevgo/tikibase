Feature: verify the configured section headings

  Background:
    Given file "tikibase.json" with content:
      """
      {
        "sections": [
          "### alpha",
          "### beta"
        ]
      }
      """
    And file "1.md" with content:
      """
      # One
      ### alpha
      [two](2.md)
      ### beta
      text
      """
    And file "2.md" with content:
      """
      # Two
      [back](1.md)
      """

  Scenario: check
    When checking
    Then it finds no issues
    And all files are unchanged

  Scenario: fix
    When fixing
    Then it finds no issues
    And all files are unchanged

  Scenario: pitstop
    When doing a pitstop
    Then it finds no issues
    And all files are unchanged
