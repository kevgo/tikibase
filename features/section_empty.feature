Feature: recognize/fix sections without content

  Background:
    Given file "test.md" with content:
      """
      # Test

      ### One

      ### Two
      [other](other.md)

      ### Three
      """
    And file "other.md" with content:
      """
      # Other
      [test](test.md)
      """

  Scenario: check
    When checking
    Then it prints:
      """
      test.md:3  section "One" has no content
      test.md:8  section "Three" has no content
      """
    And the exit code is 2

  Scenario: fix
    When fixing
    Then it prints:
      """
      test.md:3  removed empty section "One"
      test.md:8  removed empty section "Three"
      """
    And file "1.md" should contain:
      """
      # Title 1

      ### Two

      content
      """
    And the exit code is 0

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      test.md:3  removed empty section "One"
      test.md:8  removed empty section "Three"
      """
    And file "1.md" should contain:
      """
      # Title 1

      ### Two

      content
      """
    And the exit code is 0
