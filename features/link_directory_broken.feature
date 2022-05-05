Feature: recognize links to non-existing directories

  Background:
    Given file "1.md" with content:
      """
      # Title

      [zonk](non-existing/)
      <a href="non-existing/">zonk</a>
      """

  Scenario: check
    When checking
    Then it prints:
      """
      1.md:3  link to non-existing directory "non-existing"
      1.md:4  link to non-existing directory "non-existing"
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
      1.md:3  link to non-existing directory "non-existing"
      1.md:4  link to non-existing directory "non-existing"
      """
    And all files are unchanged
    And the exit code is 2
