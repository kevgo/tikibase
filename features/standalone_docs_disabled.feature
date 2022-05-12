Feature: disallow standalone documents

  Background:
    Given file "tikibase.json" with content:
      """
      {
        "standaloneDocs": false
      }
      """
    And file "1.md" with content:
      """
      # One
      """

  Scenario: check
    When checking
    Then it prints:
      """
      1.md:1  document is not connected to any other documents
      """
    And the exit code is 1

  Scenario: fix
    When fixing
    Then it finds no issues
    And all files are unchanged

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      1.md:1  document is not connected to any other documents
      """
    And the exit code is 1
