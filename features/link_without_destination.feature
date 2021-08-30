Feature: recognize/fix links to non-existing documents

  Background:
    Given file "1.md" with content:
      """
      # Title

      [Google]()
      """

  Scenario: check
    When checking
    Then it prints:
      """
      1.md:3  link without destination
      """
    And all files are unchanged
    And the exit code is 1

  Scenario: fix
    When fixing
    Then it finds no issues
    And all files are unchanged

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      1.md:3  link without destination
      """
    And all files are unchanged
    And the exit code is 1
