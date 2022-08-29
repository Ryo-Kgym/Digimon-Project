package digimon.core.status;

import lombok.Builder;
import lombok.Value;

@Builder
@Value
public class Defensive implements Status {
    private int value;
    private int max;

    private final int min = 0;
}
