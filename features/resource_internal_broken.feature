Feature: recognize/fix broken images

  Background:
    Given file "1.md" with content:
      """
      # Title

      <img src="non-existing.png">
      <img src="non-existing.png" />
      ![broken image](non-existing.png)
      """

  Scenario: check
    When checking
    Then it prints:
      """
      1.md:3  image link to non-existing file "non-existing.png"
      1.md:4  image link to non-existing file "non-existing.png"
      1.md:5  image link to non-existing file "non-existing.png"
      """
    And all files are unchanged
    And the exit code is 3

  Scenario: fix
    When fixing
    Then it finds no issues
    And all files are unchanged
    And the exit code is 0

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      1.md:3  image link to non-existing file "non-existing.png"
      1.md:4  image link to non-existing file "non-existing.png"
      1.md:5  image link to non-existing file "non-existing.png"
      """
    And all files are unchanged
    And the exit code is 3
