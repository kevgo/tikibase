Feature: recognize and fix outlier capitalization of sections

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
      [Three](3.md)
      """
    And file "3.md" with content:
      """
      # Three

      ### section
      [One](1.md)
      """

  @this
  Scenario: check
    When checking
    Then it prints:
      """
      2.md:3  section capitalization ("Section") is inconsistent with the usual form "section"
      """
    And all files are unchanged

  Scenario: fix
    When fixing
    Then it prints:
      """
      2.md:3  normalized capitalization of section "Section" to "section"
      """
    And file "2.md" should contain:
      """
      # Two

      ### Section
      [Three](3.md)
      """

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      2.md:3  section capitalization ("Section") is inconsistent with the usual form "section"
      """
    And all files are unchanged
