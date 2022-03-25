Feature: recognize sections with different capitalization

  Background:
    Given file "1.md" with content:
      """
      # One

      ### One
      [Two](2.md)

      ### one
      content
      """
    And file "2.md" with content:
      """
      # Two

      ### ONE
      [Two](2.md)
      """

  Scenario: check
    When checking
    Then it prints:
      """
      1.md:3  section title occurs with inconsistent capitalization: ONE|One|one
      1.md:6  section title occurs with inconsistent capitalization: ONE|One|one
      2.md:3  section title occurs with inconsistent capitalization: ONE|One|one
      """
    And all files are unchanged

  Scenario: fix
    When fixing
    Then it finds no issues
    And all files are unchanged

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      1.md:3  section title occurs with inconsistent capitalization: ONE|One|one
      1.md:6  section title occurs with inconsistent capitalization: ONE|One|one
      2.md:3  section title occurs with inconsistent capitalization: ONE|One|one
      """
    And all files are unchanged
