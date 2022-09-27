import {Box, Grid} from "@mui/material";
import {Attack} from "../../domain/model/attack/Attack";
import {ReactElement} from "react";

const AttackField = (props: AttackFieldProps) => {
    return (
        <Grid container sx={{
            width: '300px',
            height: '100px',
            backgroundColor: colorMap.get(props.type),
            color: 'white',
        }}>
            <Grid item xs={4}>
                {markMap.get(props.type)}
            </Grid>
            <Grid item xs={8} style={{fontSize: '60pt'}}>
                {props.attackValue}
            </Grid>
        </Grid>
    )
}

interface AttackFieldProps {
    type: AttackType
    attackValue: number
}


const AttackArea = (props: AttackAreaProps) => {
    return (
        <Box>
            <AttackField
                type={AttackType.PRIMARY}
                attackValue={props.primaryAttack.value}
            />
            <AttackField
                type={AttackType.SECONDARY}
                attackValue={props.secondaryAttack.value}
            />
            <AttackField
                type={AttackType.TERTIARY}
                attackValue={props.tertiaryAttack.value}
            />
        </Box>
    )
}
export default AttackArea


export interface AttackAreaProps {
    primaryAttack: Attack
    secondaryAttack: Attack
    tertiaryAttack: Attack
}

enum AttackType {
    PRIMARY,
    SECONDARY,
    TERTIARY,
}


const PrimaryMark = () => {
    return (
        <Box sx={{
            fontSize: '70px',
            width: '1em',
            margin: '0.1em',
        }}>
            ○
        </Box>
    )
}

const SecondaryMark = () => {
    return (
        <Box sx={{
            fontSize: '70px',
            width: '1em',
            margin: '0.1em',
        }}>
            △
        </Box>
    )
}

const TertiaryMark = () => {
    return (
        <Box sx={{
            fontSize: '70px',
            width: '1em',
            margin: '0.1em',
        }}>
            ×
        </Box>
    )
}

const markMap = new Map<AttackType, ReactElement>([
    [AttackType.PRIMARY, <PrimaryMark/>],
    [AttackType.SECONDARY, <SecondaryMark/>],
    [AttackType.TERTIARY, <TertiaryMark/>]
]);


const colorMap = new Map<AttackType, string>([
    [AttackType.PRIMARY, 'red'],
    [AttackType.SECONDARY, 'green'],
    [AttackType.TERTIARY, 'blue']
]);