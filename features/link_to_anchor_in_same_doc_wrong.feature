Feature: link to non-existing anchor in the same document

  Background:
    Given file "1.md" with content:
      """
      # One

      [wrong anchor](#zonk)
      <a href="#zonk">wrong anchor, existing file</a>
      """
    And file "2.md" with content:
      """
      # Two

      [one](1.md)
      """

  Scenario: check
    When checking
    Then it prints:
      """
      1.md:3  link to non-existing anchor "#zonk" in current file
      1.md:4  link to non-existing anchor "#zonk" in current file
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
      1.md:3  link to non-existing anchor "#zonk" in current file
      1.md:4  link to non-existing anchor "#zonk" in current file
      """
    And all files are unchanged
    And the exit code is 2
