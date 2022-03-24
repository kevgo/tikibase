Feature: Find documents without any links

  Background:
    Given file "1.md" with content:
      """
      # One

      Hello!
      """
    And file "2.md" with content:
      """
      # Two

      Hello also!
      """

  @this
  Scenario: check
    When checking
    Then it prints:
      """
      1.md:1  document is not contain any links
      2.md:1  document does not contain any links
      """
    And all files are unchanged
    And the exit code is 2

  Scenario: fix
    When fixing
    Then it finds no issues
    And all files are unchanged

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      1.md:1  document is not contain any links
      2.md:1  document does not contain any links
      """
    And all files are unchanged
    And the exit code is 2
