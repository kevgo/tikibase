Feature: recognize/fix broken images

  Background:
    Given file "1.md" with content:
      """
      # Title

      <img src="zonk.png">
      """

  Scenario: this
    When checking
    Then it prints:
      """
      1.md:3  broken image "zonk.png"
      """

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      1.md:3  broken image "zonk.png"
      """
