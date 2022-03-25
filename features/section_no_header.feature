Feature: recognize sections with an empty header

  Background:
    Given file "test.md" with content:
      """
      # Test

      ###
      [other](other.md)
      """
    And file "other.md" with content:
      """
      # Other
      [test](test.md)
      """

  Scenario: check
    When checking
    Then it prints:
      """
      test.md:3  section with empty title
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
      test.md:3  section with empty title
      """
    And all files are unchanged
    And the exit code is 1
