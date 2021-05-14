Feature: recognize/fix sections without content

  Background:
    Given file "1.md" with content:
      """
      # Title 1

      ### One

      ### Two

      content

      ### Three
      """

  Scenario: check
    When checking
    Then it prints:
      """
      1.md:3  section "One" has no content
      1.md:9  section "Three" has no content
      """
    And the exit code is 2

  Scenario: fix
    When fixing
    Then it prints:
      """
      1.md:3  removed empty section "One"
      1.md:9  removed empty section "Three"
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
      1.md:3  removed empty section "One"
      1.md:9  removed empty section "Three"
      """
    And file "1.md" should contain:
      """
      # Title 1

      ### Two

      content
      """
    And the exit code is 0
