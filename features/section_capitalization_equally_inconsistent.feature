Feature: recognize sections with equally different capitalization

  Background:
    Given file "1.md" with content:
      """
      # One

      ### section
      [Two](2.md)

      """
    And file "2.md" with content:
      """
      # Two

      ### Section
      [One](1.md)
      """

  Scenario: check
    When checking
    Then it prints:
      """
      1.md:3  section title occurs with inconsistent capitalization: Section|section
      2.md:3  section title occurs with inconsistent capitalization: Section|section
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
      1.md:3  section title occurs with inconsistent capitalization: Section|section
      2.md:3  section title occurs with inconsistent capitalization: Section|section
      """
    And all files are unchanged
