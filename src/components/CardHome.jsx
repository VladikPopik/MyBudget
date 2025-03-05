import Grid from "@mui/material/Grid";
import Card from "@mui/material/Card";
import CardActionArea from "@mui/material/CardActionArea";
import CardContent from "@mui/material/CardContent";
import CardMedia from "@mui/material/CardMedia";
import Typography from "@mui/material/Typography";
import { useNavigate } from "react-router-dom";

export default function CardHome() {
    const navigate = useNavigate();

    const handleBudget = () => {
        navigate('/budget');
    }

    return (
        <div>
            <Grid container spacing={2} sx={{ position: "absolute", top: "25%", justifyContent: "center"}}>
                {/* <Card sx={{width: "25%", marginLeft: 5, backgroundColor: "aliceblue", marginRight: 20}}>
                    <CardActionArea
                        onClick={() => handleMonitoring()}
                    >
                        <CardMedia
                        component="img"
                        height="400"
                        image="src\assets\react.svg"
                        alt="green iguana"
                        />
                        <CardContent>
                        <Typography gutterBottom variant="h5" component="div">
                            Monitoring
                        </Typography>
                        <Typography variant="body2" color="text.secondary">
                            Page for monitoring functionallity
                        </Typography>
                        </CardContent>
                    </CardActionArea>
                </Card> */}

                <Card sx={{ width: "25%", marginRight:5, backgroundColor: "aliceblue"}}>
                    <CardActionArea
                        onClick={() => handleBudget()}
                    >
                        <CardMedia
                        component="img"
                        height="400"
                        image="src\assets\react.svg"
                        alt="green iguana"
                        />
                        <CardContent>
                        <Typography gutterBottom variant="h5" component="div">
                            Budget Eval
                        </Typography>
                        <Typography variant="body2" color="text.secondary">
                            Page to evaluate Budget
                        </Typography>
                        </CardContent>
                    </CardActionArea>
                </Card>

                {/* <Card sx={{ width: "25%", marginLeft: 10, backgroundColor: "aliceblue"}}>
                    <CardActionArea
                        onClick={() => handleLight()}
                    >
                        <CardMedia
                        component="img"
                        height="400"
                        image="src\assets\react.svg"
                        alt="green iguana"
                        />
                        <CardContent>
                        <Typography gutterBottom variant="h5" component="div">
                            Light
                        </Typography>
                        <Typography variant="body2" color="text.secondary">
                            Page for light functionallity
                        </Typography>
                        </CardContent>
                    </CardActionArea>
                </Card> */}
            </Grid>
        </div>
    );
}