Feature: orphaned resource

    Background:
        Given binary file "image.png"

    Scenario: this
        When checking
        Then it prints:
            """
            orphaned image "image.png"
            """
        And all files are unchanged
