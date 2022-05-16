Feature: PDF resources

  Background:
    Given file "1.md" with content:
      """
      # Title

      Check out this [PDF file](foo.pdf).
      """
    And file "foo.pdf"

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
