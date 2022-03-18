Feature: obsolete occurrence sections

  Background:
    Given file "1.md" with content:
      """
      # One

      ### section 1

      text

      ### occurrences

      text
      """

  Scenario: check
    When checking
    Then it prints:
      """
      1.md:7  obsolete "occurrences" section
      """
    And the exit code is 1

  Scenario: fix
    When fixing
    Then it prints:
      """
      1.md:7  removed obsolete occurrences section
      """
    And file "1.md" should contain:
      """
      # One

      ### section 1

      text
      """
    And the exit code is 0

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      1.md:7  removed obsolete occurrences section
      """
    And file "1.md" should contain:
      """
      # One

      ### section 1

      text
      """
    And the exit code is 0
