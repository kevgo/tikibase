Feature: recognize/fix duplicate sections

  Background:
    Given file "2.md" with content:
      """
      # One

      ### One
      [Two](2.md)

      ### One
      content
      """
    And file "2.md" with content:
      """
      # Other
      [Two](2.md)
      """

  Scenario: check
    When checking
    Then it prints:
      """
      1.md:3  document contains multiple "One" sections
      1.md:6  document contains multiple "One" sections
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
      1.md:3  document contains multiple "One" sections
      1.md:6  document contains multiple "One" sections
      """
    And all files are unchanged
