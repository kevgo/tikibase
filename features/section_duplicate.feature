Feature: recognize/fix duplicate sections

  Background:
    Given file "1.md" with content:
      """
      # One

      ### section 1
      [Two](2.md)

      ### section 1
      content
      """
    And file "2.md" with content:
      """
      # Two
      [One](1.md)
      """

  Scenario: check
    When checking
    Then it prints:
      """
      1.md:3  document contains multiple "section 1" sections
      1.md:6  document contains multiple "section 1" sections
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
      1.md:3  document contains multiple "section 1" sections
      1.md:6  document contains multiple "section 1" sections
      """
    And all files are unchanged
