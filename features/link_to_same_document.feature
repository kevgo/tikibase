Feature: Link to the same document

  Background:
    Given file "1.md" with content:
      """
      # Title

      [myself](1.md)
      <a href="1.md">myself</a>
      """

  Scenario: check
    When checking
    Then it prints:
      """
      1.md:3  document contains link to itself
      1.md:4  document contains link to itself
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
      1.md:3  document contains link to itself
      1.md:4  document contains link to itself
      """
    And all files are unchanged
    And the exit code is 2
