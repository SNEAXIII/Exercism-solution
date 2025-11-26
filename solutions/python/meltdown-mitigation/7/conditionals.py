"""Functions to prevent a nuclear meltdown."""


def is_criticality_balanced(temperature, neutrons_emitted):
    return temperature < 800 and neutrons_emitted > 500 and temperature * neutrons_emitted < 500000


def reactor_efficiency(voltage, current, theoretical_max_power):
    efficiency = (voltage * current / theoretical_max_power) * 100
    if efficiency >= 80:
        return 'green'
    elif 80 > efficiency >= 60:
        return 'orange'
    elif 60 > efficiency >= 30:
        return 'red'
    elif 30 > efficiency:
        return 'black'


def fail_safe(temperature, neutrons_produced_per_second, threshold):
    if temperature * neutrons_produced_per_second < 0.9 * threshold:
        return 'LOW'
    if 0.9 * threshold <= temperature * neutrons_produced_per_second <= threshold * 1.1:
        return 'NORMAL'
    return 'DANGER'
