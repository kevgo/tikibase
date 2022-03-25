Feature: recognize/fix sections without content

  Background:
    Given file "1.md" with content:
      """
      # One

      ### section 1

      ### section 2
      [Two](2.md)

      ### section 3
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
      1.md:3  section "section 1" has no content
      1.md:8  section "section 3" has no content
      """
    And the exit code is 2

  Scenario: fix
    When fixing
    Then it prints:
      """
      1.md:3  removed empty section "section 1"
      1.md:8  removed empty section "section 3"
      """
    And file "1.md" should contain:
      """
      # One

      ### section 2
      [Two](2.md)
      """
    And the exit code is 0

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      1.md:3  removed empty section "section 1"
      1.md:8  removed empty section "section 3"
      """
    And file "1.md" should contain:
      """
      # One

      ### section 2
      [Two](2.md)
      """
    And the exit code is 0
