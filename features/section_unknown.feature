Feature: verify section types

  Background:
    Given file "tikibase.json" with content:
      """
      {
        "sections": [
          "what is it",
          "links"
        ]
      }
      """
    And file "1.md" with content:
      """
      # One

      ### what is it

      text

      ### zonk

      text
      """

  Scenario: check
    When checking
    Then it prints:
      """
      1.md:7  unknown section "zonk", allowed sections:
        - what is it
        - links
      """
    And the exit code is 1

  Scenario: fix
    When fixing
    Then it finds no issues
    And all files are unchanged

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      1.md:7  unknown section "zonk", allowed sections:
        - what is it
        - links
      """
    And all files are unchanged
    And the exit code is 1
