Feature: verify section types

  Background:
    Given file "tikibase.json" with content:
      """
      {
        "allowed_sections": [
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
      1.md:7  unknown section "zonk", allowed sections: what is it | links
      """

  Scenario: fix
    When fixing
    Then it prints nothing
    And all files are unchanged

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      1.md:7  unknown section "zonk", allowed sections: what is it | links
      """
    And all files are unchanged
