Feature: obsolete occurrence sections

  Background:
    Given file "1.md" with content:
      """
      # One

      ### section 1
      [Two](2.md)

      ### occurrences
      text
      """
    And file "2.md" with content:
      """
      # Other
      [One](1.md)
      """

  Scenario: check
    When checking
    Then it prints:
      """
      1.md:6  obsolete "occurrences" section
      """
    And the exit code is 1

  Scenario: fix
    When fixing
    Then it prints:
      """
      1.md:6  removed obsolete occurrences section
      """
    And file "1.md" should contain:
      """
      # One

      ### section 1
      [Two](2.md)
      """
    And the exit code is 0

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      1.md:6  removed obsolete occurrences section
      """
    And file "1.md" should contain:
      """
      # One

      ### section 1
      [Two](2.md)
      """
    And the exit code is 0
