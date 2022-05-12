Feature: link to non-existing anchor in another document

  Background:
    Given file "1.md" with content:
      """
      # One
      [link to non-existing section in 2.md](2.md#zonk)
      <a href="2.md#zonk">link to non-existing section in 2.md</a>
      """
    And file "2.md" with content:
      """
      # Two
      [backlink](1.md)
      """

  Scenario: check
    When checking
    Then it prints:
      """
      1.md:2  link to non-existing anchor "#zonk" in "2.md"
      1.md:3  link to non-existing anchor "#zonk" in "2.md"
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
      1.md:2  link to non-existing anchor "#zonk" in "2.md"
      1.md:3  link to non-existing anchor "#zonk" in "2.md"
      """
    And all files are unchanged
    And the exit code is 2
